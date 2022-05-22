## 術語列表

```
SOP => Start Of Parse
EOP => End Of Parse
ANY => Any String
SELF => Self Parser
A | B => A or B
(...) => Group Parse
[...] => Group Template Parse
!A => Not A
"..." => String
```

## 語法設計

```
begin = " " | SOP
end = " " | EOP

inline = [(begin s_tag) inner (e_tag end)]

bold = inline as {
    s_tag = "*"
    e_tag = "*"
    inner = !SELF inline
}

italic = inline as {
    s_tag = "/"
    e_tag = "/"
    inner = !SELF inline
}

underline = inline as {
    s_tag = "_"
    e_tag = "_"
    inner = !SELF inline
}

linethrough = inline as {
    s_tag = "-"
    e_tag = "-"
    inner = !SELF inline
}
```