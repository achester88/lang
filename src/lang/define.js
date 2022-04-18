Expr {
    type_of: "apply",
    operator: Some(
        Expr {
            type_of: "word",
            operator: None,
            args: None,
            value: Some(
                "do",
            ),
        },
    ),
    args: Some(
        [
            Expr {
                type_of: "apply",
                operator: Some(
                    Expr {
                        type_of: "word",
                        operator: None,
                        args: None,
                        value: Some(
                            "define",
                        ),
                    },
                ),
                args: Some(
                    [
                        Expr {
                            type_of: "word",
                            operator: None,
                            args: None,
                            value: Some(
                                "total",
                            ),
                        },
                        Expr {
                            type_of: "value",
                            operator: None,
                            args: None,
                            value: Some(
                                "0",
                            ),
                        },
                    ],
                ),
                value: None,
            },
            Expr {
                type_of: "apply",
                operator: Some(
                    Expr {
                        type_of: "word",
                        operator: None,
                        args: None,
                        value: Some(
                            "define",
                        ),
                    },
                ),
                args: Some(
                    [
                        Expr {
                            type_of: "word",
                            operator: None,
                            args: None,
                            value: Some(
                                "count",
                            ),
                        },
                        Expr {
                            type_of: "value",
                            operator: None,
                            args: None,
                            value: Some(
                                "1",
                            ),
                        },
                    ],
                ),
                value: None,
            },
            Expr {
                type_of: "apply",
                operator: Some(
                    Expr {
                        type_of: "word",
                        operator: None,
                        args: None,
                        value: Some(
                            "while",
                        ),
                    },
                ),
                args: Some(
                    [
                        Expr {
                            type_of: "apply",
                            operator: Some(
                                Expr {
                                    type_of: "word",
                                    operator: None,
                                    args: None,
                                    value: Some(
                                        "<",
                                    ),
                                },
                            ),
                            args: Some(
                                [
                                    Expr {
                                        type_of: "word",
                                        operator: None,
                                        args: None,
                                        value: Some(
                                            "count",
                                        ),
                                    },
                                    Expr {
                                        type_of: "value",
                                        operator: None,
                                        args: None,
                                        value: Some(
                                            "11",
                                        ),
                                    },
                                ],
                            ),
                            value: None,
                        },
                        Expr {
                            type_of: "apply",
                            operator: Some(
                                Expr {
                                    type_of: "word",
                                    operator: None,
                                    args: None,
                                    value: Some(
                                        "do",
                                    ),
                                },
                            ),
                            args: Some(
                                [
                                    Expr {
                                        type_of: "apply",
                                        operator: Some(
                                            Expr {
                                                type_of: "word",
                                                operator: None,
                                                args: None,
                                                value: Some(
                                                    "define",
                                                ),
                                            },
                                        ),
                                        args: Some(
                                            [
                                                Expr {
                                                    type_of: "word",
                                                    operator: None,
                                                    args: None,
                                                    value: Some(
                                                        "total",
                                                    ),
                                                },
                                                Expr {
                                                    type_of: "apply",
                                                    operator: Some(
                                                        Expr {
                                                            type_of: "word",
                                                            operator: None,
                                                            args: None,
                                                            value: Some(
                                                                "+",
                                                            ),
                                                        },
                                                    ),
                                                    args: Some(
                                                        [
                                                            Expr {
                                                                type_of: "word",
                                                                operator: None,
                                                                args: None,
                                                                value: Some(
                                                                    "total",
                                                                ),
                                                            },
                                                            Expr {
                                                                type_of: "word",
                                                                operator: None,
                                                                args: None,
                                                                value: Some(
                                                                    "count",
                                                                ),
                                                            },
                                                        ],
                                                    ),
                                                    value: None,
                                                },
                                            ],
                                        ),
                                        value: None,
                                    },
                                    Expr {
                                        type_of: "apply",
                                        operator: Some(
                                            Expr {
                                                type_of: "word",
                                                operator: None,
                                                args: None,
                                                value: Some(
                                                    "define",
                                                ),
                                            },
                                        ),
                                        args: Some(
                                            [
                                                Expr {
                                                    type_of: "word",
                                                    operator: None,
                                                    args: None,
                                                    value: Some(
                                                        "count",
                                                    ),
                                                },
                                                Expr {
                                                    type_of: "apply",
                                                    operator: Some(
                                                        Expr {
                                                            type_of: "word",
                                                            operator: None,
                                                            args: None,
                                                            value: Some(
                                                                "+",
                                                            ),
                                                        },
                                                    ),
                                                    args: Some(
                                                        [
                                                            Expr {
                                                                type_of: "word",
                                                                operator: None,
                                                                args: None,
                                                                value: Some(
                                                                    "count",
                                                                ),
                                                            },
                                                            Expr {
                                                                type_of: "value",
                                                                operator: None,
                                                                args: None,
                                                                value: Some(
                                                                    "1",
                                                                ),
                                                            },
                                                        ],
                                                    ),
                                                    value: None,
                                                },
                                            ],
                                        ),
                                        value: None,
                                    },
                                ],
                            ),
                            value: None,
                        },
                    ],
                ),
                value: None,
            },
            Expr {
                type_of: "apply",
                operator: Some(
                    Expr {
                        type_of: "word",
                        operator: None,
                        args: None,
                        value: Some(
                            "print",
                        ),
                    },
                ),
                args: Some(
                    [
                        Expr {
                            type_of: "word",
                            operator: None,
                            args: None,
                            value: Some(
                                "total",
                            ),
                        },
                    ],
                ),
                value: None,
            },
        ],
    ),
    value: None,
}