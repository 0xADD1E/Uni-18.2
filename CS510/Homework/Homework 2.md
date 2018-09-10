# Homework 2

Addie Morrison, Q688R999

### 1A

```
expr    -> expr + term | term
term    -> term % newterm | term * newterm | newterm
newterm -> factor ^ newterm | factor
factor  -> ( expr ) | number
number  -> number digit | digit
digit   -> 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9
```

### 1B

```
expr    -> term { + term }
term    -> newterm { % newterm | * newterm }
newterm -> factor { ^ factor}
factor  -> ( expr ) | number
number  -> digit { digit }
digit   -> 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9
```

## 2

*Note: trees are Left -> Right instead of Top -> Bot to save paper*

### 2A

![Tree 2A](Homework 2 Assets\Tree 2A.svg)

### 2B

![Tree 2B](Homework 2 Assets\Tree 2B.svg)

### 2C

![Tree 2C](Homework 2 Assets\Tree 2C.svg)

### 2D

![Tree 2D](Homework 2 Assets\Tree 2D.svg)

### 2E

![Tree 2E](Homework 2 Assets\Tree 2E.svg)

## 3

It would solve the left recursion issue, but the two are not equivelent

## 4

We want it to terminate on newline, instead of continuing (ie: `match` calls `getToken`)

## 5

It simply wouldn't terminate on `\n`

## 6

Multiple possible leftmost derivations of the same input are what results in ambiguous grammars

## 7

Create another rule to disambiguate


