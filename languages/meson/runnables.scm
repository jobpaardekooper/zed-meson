; Meson executable targets whose names are string literals.
;
; Dynamic target names (for example, executable(target_name, ...)) cannot be
; turned into task labels without evaluating the Meson program, so they are
; intentionally not tagged here.
(
  (normal_command
    command: (identifier) @_command
    .
    (variableunit
      (string) @run @meson_target))
  (#eq? @_command "executable")
  (#match? @meson_target "^'[^'@]*'$")
  (#set! tag meson-executable)
)
