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

let state =
  "./solutions/day-02/input.txt"
  |> Core.In_channel.read_lines
  |> List.hd
  |> String.split_on_char(',')
  |> List.map(int_of_string)
  |> Array.of_list;

let runAddition = position => {
  state[state[position + 3]] =
    state[state[position + 1]] + state[state[position + 2]];
};

let runMultiplication = position =>
  state[state[position + 3]] =
    state[state[position + 1]] * state[state[position + 2]];

let rec run = position => {
  let command = commandFromInt(state[position]);
  switch (command) {
  | Add =>
    runAddition(position);
    run(position + 4);
  | Multiply =>
    runMultiplication(position);
    run(position + 4);
  | _ => ()
  };
};

state[1] = 12;
state[2] = 2;
run(0);

Printf.printf("Position 0: %d\n", state[0]);