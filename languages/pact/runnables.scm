(s_expression
  head: (s_expression_head) @name
  (#any-of? @name
   "begin-tx" "expect" "expect-failure" "expect-that" "bench" "verify" "typecheck"
  )
  tail: (string) @run
  (#set! tag pact-repl)
) @pact-repl
