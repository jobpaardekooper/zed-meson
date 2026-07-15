use zed_extension_api::{
    self as zed, BuildTaskDefinition, BuildTaskDefinitionTemplatePayload, DebugRequest,
    DebugScenario, LaunchRequest, TaskTemplate,
};

pub const LOCATOR_NAME: &str = "meson";

const BUILD_DIR_ENV: &str = "ZED_MESON_BUILD_DIR";
const COMMAND_ENV: &str = "ZED_MESON_COMMAND";
const DEFINED_IN_ENV: &str = "ZED_MESON_DEFINED_IN";
const TARGET_ENV: &str = "ZED_MESON_TARGET";

pub fn create_debug_scenario(
    locator_name: &str,
    build_task: TaskTemplate,
    resolved_label: String,
    debug_adapter_name: String,
) -> Option<DebugScenario> {
    if locator_name != LOCATOR_NAME || env_value(&build_task, TARGET_ENV).is_none() {
        return None;
    }

    Some(DebugScenario {
        label: resolved_label.replacen("build", "debug", 1),
        adapter: debug_adapter_name,
        build: Some(BuildTaskDefinition::Template(
            BuildTaskDefinitionTemplatePayload {
                locator_name: Some(LOCATOR_NAME.to_string()),
                template: build_task,
            },
        )),
        config: "{}".to_string(),
        tcp_connection: None,
    })
}

pub fn locate_debug_target(
    locator_name: &str,
    build_task: TaskTemplate,
) -> Result<DebugRequest, String> {
    if locator_name != LOCATOR_NAME {
        return Err(format!("unknown Meson debug locator: {locator_name}"));
    }

    let build_dir = required_env_value(&build_task, BUILD_DIR_ENV)?;
    let meson_command = required_env_value(&build_task, COMMAND_ENV)?;
    let target_name = unquote_meson_string(&required_env_value(&build_task, TARGET_ENV)?)?;
    let defined_in = required_env_value(&build_task, DEFINED_IN_ENV)?;

    let output = zed::process::Command::new(meson_command)
        .args(["introspect", "--targets", build_dir.as_str()])
        .envs(build_task.env.clone())
        .output()
        .map_err(|error| format!("failed to run Meson introspection: {error}"))?;

    if output.status != Some(0) {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "Meson could not inspect build directory {build_dir}: {}",
            stderr.trim()
        ));
    }

    let program = executable_from_introspection(&output.stdout, &target_name, &defined_in)?;
    Ok(DebugRequest::Launch(LaunchRequest {
        program,
        cwd: build_task.cwd,
        args: Vec::new(),
        envs: Vec::new(),
    }))
}

fn env_value<'a>(task: &'a TaskTemplate, name: &str) -> Option<&'a str> {
    task.env
        .iter()
        .find_map(|(key, value)| (key == name).then_some(value.as_str()))
}

fn required_env_value(task: &TaskTemplate, name: &str) -> Result<String, String> {
    env_value(task, name)
        .map(ToOwned::to_owned)
        .ok_or_else(|| format!("Meson debug task is missing {name}"))
}

fn unquote_meson_string(value: &str) -> Result<String, String> {
    let value = value.trim();
    if let Some(inner) = value
        .strip_prefix("'''")
        .and_then(|value| value.strip_suffix("'''"))
    {
        return Ok(inner.to_string());
    }
    if let Some(inner) = value
        .strip_prefix('\'')
        .and_then(|value| value.strip_suffix('\''))
    {
        return Ok(inner.to_string());
    }
    Err(format!(
        "Meson executable target must have a literal name, got {value}"
    ))
}

fn executable_from_introspection(
    stdout: &[u8],
    target_name: &str,
    defined_in: &str,
) -> Result<String, String> {
    let targets: zed::serde_json::Value = zed::serde_json::from_slice(stdout)
        .map_err(|error| format!("invalid output from meson introspect --targets: {error}"))?;
    let targets = targets
        .as_array()
        .ok_or_else(|| "Meson target introspection did not return an array".to_string())?;

    let mut matching_name = None;
    for target in targets {
        if target.get("name").and_then(|value| value.as_str()) != Some(target_name)
            || target.get("type").and_then(|value| value.as_str()) != Some("executable")
        {
            continue;
        }

        let filename = target
            .get("filename")
            .and_then(|value| value.as_array())
            .and_then(|filenames| filenames.first())
            .and_then(|value| value.as_str());
        let Some(filename) = filename else {
            continue;
        };

        if target.get("defined_in").and_then(|value| value.as_str()) == Some(defined_in) {
            return Ok(filename.to_string());
        }
        matching_name = Some(filename.to_string());
    }

    matching_name.ok_or_else(|| {
        format!("Meson introspection did not report executable target {target_name}")
    })
}
