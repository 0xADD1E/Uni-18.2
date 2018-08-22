# Relations

Let $A$ and $B$ be sets

A binary relation from $A$ to $B$ is a subset of $A\times B$

## Notation

If $R$ is a relation from $A$ to $B$, and $a\in A$ and $b\in B$, we write $a R b$ ("$a$ is related to $b$"), or $(a,b)\in R$

If $a$ is not related to $b$, we write $a\cancel{R} b$

## Examples

### WSU ID and Name

$(ABC123, John)\in R$

### Math Functions

(where $f(x)=x^2; f: \Bbb N\to \Bbb N$)

- $f(2)=4 \to (2,4)\in R$

- $f(3)=9 \to (3,9)\in R$

### Family

(where John is J, Eric is E, Sarah is S, all are siblings)

$\{(J,E),(E,J),(S,E),(E,S),(S,J),(J,S)\}$

### More people

$R$ is from the set of people in this room, to the set of people in this room, such that $(a,b)\in R$, and have the same first initial

$R=\{(John, Jon),(Jon,John),(Blake,Brandon),(Brandon,Blake),...\}$

### Enumerating $\le$ for $\{1,2,3,4\}$

$R=\{(1,2),(2,2),(1,3),(2,3),(3,3),(1,4),(2,4),(3,4),(4,4)\}$

### Enumerating "divides" for $\{1,2,3,4\}$

$R=\{(1,1),(1,2),(1,3),(1,4),(2,2),(2,4),(3,3),(4,4)\}$

## Properties

A relation $R$ (defined from $A$ to $A$) is *reflexive* if $(a,a)\in R$ for every $a\in A$

- The sibling relation *is not* reflexive (IE: you are not your own sibling)
- Equality *is* reflexive (IE: $1=1$)
- So is divisibility (at least over $\Bbb Z^+$, $x\mid x$)

A relation (defined from $A$ to $A$) is *symmetric* if $\forall a\forall b\,(b,a)\in R \iff (a,b)\in R$, 

- Our sibling relation *is* symmetric (If someone is your sibiling, you are theirs)
- So is equality for $\Bbb R$
- $\le$ *is not* symmetric ($1\le 4$, but $4\cancel{\le}1$)

*Antisymmetric* relations are *not* necessarily the opposite

$\forall a\forall b\,((b,a)\in R \iff (a,b)\in R)\to a=b$

- This means equality **is both *symmetric* and *antisymmetric***
- For a less cheaty example, $\le​$ is *antisymmetric*

A relation $R$ (defined from $A$ to $A$) is *transitive* if $((a,b)\in R)\and(b,c)\in R)\to (a,c)\in R$

- Divides, $\le$, **EQUALITY** :frowning: for $\Bbb Z^+$
- "is a decendent of..." (I am a decendent of my parent, my parent is a decenent of my grandparent, I am a decendent of my grandparent)

