# AI Usage Log

## 2026-03-18

Date: 2026-03-18
Commit: 56a2bedd5fafbf5567e34277468980f1cea0e458

Asked Claude to brainstorm some core modeling optimizations.
The only one I found worthwhile was using a bitset to represent the tile possibilities, which is a great optimization.
It lets us store all the possibilities in a single 16 bit int and use bitwise operations which are extremely quick at runtime.
The alternative would be to use a `Set<Digit>` which is WAY less efficient:w
