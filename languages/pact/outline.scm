(module
    "module" @context
    name: (_) @name
    governance: (module_governance) @context) @item

(defun
    "defun" @context
    name: (_) @name) @item
(defcap
    "defcap" @context
    name: (_) @name) @item
(defpact
    "defpact" @context
    name: (_) @name) @item

(defschema
    "defschema" @context
    name: (_) @name) @item
(deftable
    "deftable" @context
    name: (_) @name
    schema: (_) @context ) @item
(defconst
    "defconst" @context
    name: (_) @name) @item

(interface
    "interface" @context
    name: (_) @name) @item

(s_expression
    head: (s_expression_head) @name
    tail: (string) @context
    (#any-of? @name
        "begin-tx" "expect" "expect-failure" "expect-that" "bench" "verify" "typecheck"
    )
) @item
