# TODO: Finish Tokens

Tokens are currently just strings. They should instead be a custom struct with the following properties:
- `value` (string)
- `position` (tuple)
  - `line`
  - `col` (starting col)

All errors should then use the position of a token to print the location of the error.