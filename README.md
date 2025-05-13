## Todo

- [ ] Add command that reads a solution file and puts the relevant content to the clipboard so that the user can directly paste it to leetcode
  - [ ] We should remove every attribute `#[cfg(test)]`
  - [ ] If there is a line `struct Solution;` it means it is a function problem and we should remove it
  - [ ] If there is a test module (`mod tests`), we should remove it entirely (be aware of `{` within comments, and comments can be `//`, `///` or `/**`)
- [ ] When pasting a default code from leetcode, we may should clean up the comments as they can be meant to define something (see pb 1865)
- [ ] Random functions should maybe not have test modules (see pb 470)
- [ ] We should not add the pattern TEST_COMPILER_CONFIGURATION_ATTRIBUTE to lines within comments (see pb 86)
- [ ] Add the test module to the generated solution file
  - [ ] We should interpret the example testcases -> if there are `n` params, it seems they are grouped as `n` elements followed by `\n` (if there are `m` test cases then there are `m` groups of `n` elements)
  - [ ] The expected return is not defined in the example testcases, we should decide what to write (a `todo!` macro ?)
- [ ] We should refactor the Json names in the `fetch` script