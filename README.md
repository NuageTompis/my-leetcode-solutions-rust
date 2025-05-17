## Todo

- [ ] Add command that reads a solution file and puts the relevant content to the clipboard so that the user can directly paste it to leetcode
  - [ ] We should remove every attribute `#[cfg(test)]`
  - [ ] If there is a line `struct Solution;` it means it is a function problem and we should remove it
  - [ ] If there is a test module (`mod tests`), we should remove it entirely (be aware of `{` within comments, and comments can be `//`, `///` or `/**`)
- [ ] ~~When pasting a default code from leetcode, we may should clean up the comments as they can be meant to define something in rust (see pb 1865)~~
  > There should be a test module ending the file anyways, so there should not be a problem with comments not defining anything
- [ ] Random functions should maybe not have test modules (see pb 470)
- [x] We should not add the pattern TEST_COMPILER_CONFIGURATION_ATTRIBUTE to lines within comments (see pb 86)
- [ ] Add the test module to the generated solution file
  - [ ] We should interpret the example testcases
    - [ ] Handle function attributes modifiers (`&,` `&mut`)
    - [ ] Handle `ListNode` and `TreeNode` data types
  - [x] The expected return is not defined in the example testcases, we should decide what to write (a `todo!` macro ?)
  - Progress
    - [x] Done for *Function* problems (except elements listed above)
    - [ ] Todo for *Class* problems
- [ ] We should refactor the Json names in the `fetch` script