let commandCount = ref(0);

type command =
  | Add
  | Multiply
  | Input
  | Output
  | Halt;

type parameterMode =
  | Position
  | Immediate;

exception UnknownCommand;
exception UnknownParameterMode;

let parameterModeFromInt = int => {
  switch (int) {
  | 0 => Position
  | 1 => Immediate
  | _ => raise(UnknownCommand)
  };
};

let parameterModesFromList = (intList, length) => {
  List.init(length, i =>
    switch (List.nth_opt(intList, i)) {
    | Some(int) => parameterModeFromInt(int)
    | None => parameterModeFromInt(0)
    }
  );
};

let commandFromInt = (int, paramModes) => {
  switch (int) {
  | 1 => (Add, parameterModesFromList(paramModes, 3))
  | 2 => (Multiply, parameterModesFromList(paramModes, 3))
  | 3 => (Input, [Position])
  | 4 => (Output, parameterModesFromList(paramModes, 1))
  | 99 => (Halt, [])
  | _ => raise(UnknownCommand)
  };
};

let getPointer = (position, parameterMode, state) => {
  let pointer =
    switch (parameterMode) {
    | Position => state[position]
    | Immediate => position
    };
  pointer;
};

let runAddition = (position, parameterModes, state) => {
  let pMode = List.nth(parameterModes);
  let in1 = getPointer(position + 1, pMode(0), state);
  let in2 = getPointer(position + 2, pMode(1), state);
  let out = getPointer(position + 3, pMode(2), state);
  state[out] = state[in1] + state[in2];
};

let runMultiplication = (position, parameterModes, state) => {
  let pMode = List.nth(parameterModes);
  let in1 = getPointer(position + 1, pMode(0), state);
  let in2 = getPointer(position + 2, pMode(1), state);
  let out = getPointer(position + 3, pMode(2), state);
  state[out] = state[in1] * state[in2];
};

let runInput = (position, state) => {
  let out = getPointer(position + 1, Position, state);
  state[out] = 1;
};

let runOutput = (position, parameterModes, state) => {
  let pMode = List.nth(parameterModes);
  Printf.printf(
    "Output: %d\n",
    state[getPointer(position + 1, pMode(0), state)],
  );
};

let rec runCommand = (position, state) => {
  commandCount := commandCount^ + 1;
  let digits = state[position] |> AdventOfCode.Util.Int.digits;
  let command =
    switch (List.rev(digits)) {
    | [i] => commandFromInt(i, [])
    | [ones, tens, ...revParamModes] =>
      commandFromInt(ones + tens * 10, revParamModes)
    | _ => raise(UnknownCommand)
    };
  switch (command) {
  | (Add, parameterModes) =>
    runAddition(position, parameterModes, state);
    runCommand(position + 4, state);
  | (Multiply, parameterModes) =>
    runMultiplication(position, parameterModes, state);
    runCommand(position + 4, state);
  | (Input, _) =>
    runInput(position, state);
    runCommand(position + 2, state);
  | (Output, parameterModes) =>
    runOutput(position, parameterModes, state);
    runCommand(position + 2, state);
  | _ => Printf.printf("Halting at %d\n", position)
  };
};

let input =
  "./solutions/day-05/input.txt"
  |> Core.In_channel.read_lines
  |> List.hd
  |> String.split_on_char(',')
  |> List.map(int_of_string);

let state = Array.of_list(input);
runCommand(0, state);