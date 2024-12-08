# Part 2
Taking the 4 surrounding characters of the 'A' in this order:
```
1 2
 A
3 4
```

so that 
```
M S
 A
M S 
```
is MSMS

Valid combinations are:
```
M M   MMSS
 A
S S

S M   SMSM
 A
S M

M S   MSMS
 A
M S

S S   SSMM
 A
M M
```

Invalid combinations are:
```
S M   SMMS
 A
M S

M S   MSSM
 A
S M
```
