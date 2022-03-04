# base2991

A calculator that can do floating-point operations in 4 different bases: decimal, binary, octal, and hexadecimal. It supports base conversions and arithmetic operations (addition, subtraction, multiplication, and divison).

## Example session

```
> 5d+3d
8.0000000000_10
> 5d*3d_b
1111.0000000000_2
> 5d/3d_o
1.5252525253_8
> 3h+30d*50o-100b
4af.0000000000_16
> baf.bafh_d
2991.7302246094_10
```

## Grammar
Below is a skeleton of base2991's grammar. View the full LALRPOP grammar at [src/parser.lalrpop](https://github.com/yaxollum/base2991/blob/main/src/parser.lalrpop).

```
Ast = {
    (),
    BaseConv
}

BaseConv = {
    <Add> "_" r"[xh]",
    <Add> "_" "b",
    <Add> "_" "o",
    <Add> "_" "d",
    Add
}

Add = {
    <a:Mul> "+" <b:Add>,
    <a:Mul> "-" <b:Add>,
    Mul
}

Mul = {
    <a:Num> "*" <b:Mul>,
    <a:Num> "/" <b:Mul>,
    Num
}

Num = r"[0-9a-fA-F]+(\.[0-9a-fA-F]+)?[bdoxh]"
```
