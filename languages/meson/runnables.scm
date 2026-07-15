; Meson executable targets whose names are string literals and which do not
; override the output suffix.
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
  (#not-match? @_executable "name_suffix[[:space:]]*:")
  (#set! tag meson-executable)
)

; A non-empty literal name_suffix is part of Meson's target selector and output filename.
; Keep these targets separate so same-named executables can be selected
; unambiguously. A dynamic or empty name_suffix cannot be resolved into this
; selector template, so it is intentionally not tagged.
(
  (normal_command
    command: (identifier) @_command
    .
    (variableunit
      (string) @run @meson_target)
    (pair
      key: (identifier) @_suffix_key
      value: (string) @meson_suffix))
  (#eq? @_command "executable")
  (#eq? @_suffix_key "name_suffix")
  (#match? @meson_target "^'[^'@]*'$")
  (#match? @meson_suffix "^'[^'@]+'$")
  (#set! tag meson-executable-suffixed)
)
