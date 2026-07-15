; Meson executable targets whose names are string literals and which do not
; override the output prefix or suffix.
;
; Dynamic target names (for example, executable(target_name, ...)) cannot be
; turned into task labels without evaluating the Meson program, so they are
; intentionally not tagged here.
(
  (normal_command
    command: (identifier) @_command
    .
    (variableunit
      (string) @run @meson_target)) @_executable
  (#eq? @_command "executable")
  (#match? @meson_target "^'[^'@]*'$")
  (#not-match? @_executable "name_prefix[[:space:]]*:")
  (#not-match? @_executable "name_suffix[[:space:]]*:")
  (#set! tag meson-executable)
)

; A non-empty literal name_suffix is part of Meson's target selector and output filename.
; Keep these targets separate so same-named executables can be selected
; unambiguously. A dynamic or empty name_suffix cannot be resolved into this
; selector template, so it is intentionally not tagged. name_prefix is handled
; by separate templates below because it changes only the output filename.
(
  (normal_command
    command: (identifier) @_command
    .
    (variableunit
      (string) @run @meson_target)
    (pair
      key: (identifier) @_suffix_key
      value: (string) @meson_suffix)) @_executable
  (#eq? @_command "executable")
  (#eq? @_suffix_key "name_suffix")
  (#match? @meson_target "^'[^'@]*'$")
  (#match? @meson_suffix "^'[^'@]+'$")
  (#not-match? @_executable "name_prefix[[:space:]]*:")
  (#set! tag meson-executable-suffixed)
)

; A literal name_prefix changes the output filename, but not Meson's target
; selector. Capture it for the run command and debugger target lookup.
(
  (normal_command
    command: (identifier) @_command
    .
    (variableunit
      (string) @run @meson_target)
    (pair
      key: (identifier) @_prefix_key
      value: (string) @meson_prefix)) @_executable
  (#eq? @_command "executable")
  (#eq? @_prefix_key "name_prefix")
  (#match? @meson_target "^'[^'@]*'$")
  (#match? @meson_prefix "^'[^'@]*'$")
  (#not-match? @_executable "name_suffix[[:space:]]*:")
  (#set! tag meson-executable-prefixed)
)

; name_prefix before name_suffix.
(
  (normal_command
    command: (identifier) @_command
    .
    (variableunit
      (string) @run @meson_target)
    (pair
      key: (identifier) @_prefix_key
      value: (string) @meson_prefix)
    (pair
      key: (identifier) @_suffix_key
      value: (string) @meson_suffix))
  (#eq? @_command "executable")
  (#eq? @_prefix_key "name_prefix")
  (#eq? @_suffix_key "name_suffix")
  (#match? @meson_target "^'[^'@]*'$")
  (#match? @meson_prefix "^'[^'@]*'$")
  (#match? @meson_suffix "^'[^'@]+'$")
  (#set! tag meson-executable-prefixed-suffixed)
)

; name_suffix before name_prefix.
(
  (normal_command
    command: (identifier) @_command
    .
    (variableunit
      (string) @run @meson_target)
    (pair
      key: (identifier) @_suffix_key
      value: (string) @meson_suffix)
    (pair
      key: (identifier) @_prefix_key
      value: (string) @meson_prefix))
  (#eq? @_command "executable")
  (#eq? @_prefix_key "name_prefix")
  (#eq? @_suffix_key "name_suffix")
  (#match? @meson_target "^'[^'@]*'$")
  (#match? @meson_prefix "^'[^'@]*'$")
  (#match? @meson_suffix "^'[^'@]+'$")
  (#set! tag meson-executable-prefixed-suffixed)
)
