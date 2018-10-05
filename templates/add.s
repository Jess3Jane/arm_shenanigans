# adds register a and register b
# result in a, carry in b
# these must be 3 different registers
# uses 8 bytes of temp

mov !c, !tmp
str !a, [!c]
str !b, [!c, #4]

ldrb !a, [!c]
ldrb !b, [!c, #4]
ldrb !b, [!a, !b]!
ldrb !b, [!c, #1]
strb !a, [!c, #4]
strh !a, [!c]

ldrb !a, [!c, #1]
ldrb !b, [!a, !b]!
ldrb !b, [!c, #5]
ldrb !b, [!a, !b]!
strb !a, [!c, #5]
strh !a, [!c]

ldrb !b, [!c, #1]
ldrb !a, [!c, #2]
ldrb !b, [!a, !b]!
ldrb !b, [!c, #6]
ldrb !b, [!a, !b]!
strb !a, [!c, #6]
strh !a, [!c]

ldrb !b, [!c, #1]
ldrb !a, [!c, #3]
ldrb !b, [!a, !b]!
ldrb !b, [!c, #7]
ldrb !b, [!a, !b]!
strb !a, [!c, #7]
strh !a, [!c]

ldr  !a, [!c, #4]
ldrb !b, [!c, #1]
