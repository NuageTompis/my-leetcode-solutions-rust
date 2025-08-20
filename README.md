## Motivation

As I began my journey on Leetcode, I wasn't fond of the online text editor, and quickly switched to my favorite IDE, copy-pasting my code everytime I wanted to test it. This, coupled with the limited options on the testcases, the desire to have a repository containing all my scripts, and the fact that each and every test required traffic on the Internet and thus unnecessary energy consumption, fueled me with the desire to develop an alternative.

## Description

Leetcrust is a tool whose role is to fetch a problem's *default code* from leetcode and automatically construct a test module with the exemple testcases described by leetcode.

You can then start coding, focusing (almost entirely) on your solution, and test it via `cargo test` and the generated test module.

Once you're ready to submit it, you can run a command to place your code to your clipboard and then head back to leetcode to paste it.

## Usage

### Recommended: define your preferences

- Tell leetcrust if you are a premium leetcode user by running `cargo conf premium <0 or 1>`
  > Note that paid-only problems aren't yet supported so this simply aborts early when trying to create a solution file for such problems
- Tell leetcrust how you'd like to escape Rust's deadcode warnings. This is done by either:
  - Using the `#[allow(deadcode)]` attribute on a solution file (rust module)
  - Adding the `#[cfg(test)]` attribute to every function, struct declaration, and import
  > While the second approach may feel cumbersome, it provides additional warnings when you write some code that you forget to use in your tests. This is why it is the default. You should run `cargo conf allow-dead-code <0 or 1>` to configure it

### The fun stuff: create your first solution

- Find a problem you'd like to solve (for example problem number `1`, [Two Sum](https://leetcode.com/problems/two-sum/description/))
- Run `cargo create 1`
- Head to the generated file (click on the command output if your terminal supports this feature)
- The command generated a test module alongside the default code, with one test function for every example testcase provided by leetcode. However, the api doesn't provide the expected output for each input, so you'll have to fill in this value
- Write your implementation of the solution
- Test your solution by running `cargo test 1` (or `cargo test s1_` once you start having many solution files)
- Run `cargo clip 1` to copy your solution
- Head back to leetcode, and replace the whole default code with `ctrl+v`
- Submit

#### Demo

![demo](.docs/leetcrust-demo.gif)

## Advancement

| Task                                                                                      | Progress           |
| ----------------------------------------------------------------------------------------- | ------------------ |
| Write script that fetchs a problem's info from leetcode's api                             | :white_check_mark: |
| Write script that creates a test module for a given problem                               | :white_check_mark: |
| Write command that creates a solution file with a test module                             | :white_check_mark: |
| Write command that reads a solution file and places the relevant part to the clipboard    | :white_check_mark: |
| Write command to configure user preferences                                               | :white_check_mark: |
| Handle paid-only problems for premium users                                               | `TODO`             |
| Handle function attributes modifiers in the test module (`&,` `&mut`) (may be unfeasible) | `TODO`             |
| Add a `verbose` flag to print detailed messages                                           | `TODO`             |
| We should refactor the Json names in the `fetch` script                                   | `TODO`             |
| Random functions should maybe not have test modules (see pb 470)                          | `TODO`             |
| Either implement or remove the `fetch` command                                            | `TODO`             |
| Write a basic documentation for the repo                                                  | :white_check_mark: |
| Add a gif to the readme with the workflow detailed in [Usage](#usage)                     | :white_check_mark: |

## Contributing

First of all, thank you for considering to contribute on this project. If you'd like to help me implement the tasks listed above, of if you found some mistakes in the project, please feel free to submit a pull request on GitHub.

## Credits

- This project is inspired from [this repo](https://github.com/aylei/leetcode-rust)

- [This repo](https://github.com/alfaarghya/alfa-leetcode-api/tree/main) was very helpful to find info in leetcode's api endpoints