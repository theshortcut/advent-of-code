type command =
  | Add
  | Multiply
  | Halt;

exception UnknownCommand;

let commandFromInt = int => {
  switch (int) {
  | 1 => Add
  | 2 => Multiply
  | 99 => Halt
  | _ => raise(UnknownCommand)
  };
};

let input =
  "./solutions/day-02/input.txt"
  |> Core.In_channel.read_lines
  |> List.hd
  |> String.split_on_char(',')
  |> List.map(int_of_string);

let runAddition = (position, state) => {
  state[state[position + 3]] =
    state[state[position + 1]] + state[state[position + 2]];
};

let runMultiplication = (position, state) =>
  state[state[position + 3]] =
    state[state[position + 1]] * state[state[position + 2]];

let rec runCommand = (position, state) => {
  let command = commandFromInt(state[position]);
  switch (command) {
  | Add =>
    runAddition(position, state);
    runCommand(position + 4, state);
  | Multiply =>
    runMultiplication(position, state);
    runCommand(position + 4, state);
  | _ => ()
  };
};

let runProgram = (noun, verb) => {
  let state = Array.of_list(input);
  state[1] = noun;
  state[2] = verb;
  runCommand(0, state);
  state[0];
};

for (noun in 0 to 99) {
  for (verb in 0 to 99) {
    let output = runProgram(noun, verb);
    if (output == 19690720) {
      Printf.printf("Solution: %d\n", 100 * noun + verb);
    };
  };
};