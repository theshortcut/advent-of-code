type direction =
  | Up
  | Down
  | Left
  | Right;

exception UnknownDirection;

type coord = (int, int);

module CoordMap = Map.Make(String);

let stringToDirection = str => {
  let length = String.length(str);
  let dirChar = str.[0];
  let distance = String.sub(str, 1, length - 1) |> int_of_string;
  switch (dirChar) {
  | 'U' => (Up, distance)
  | 'D' => (Down, distance)
  | 'L' => (Left, distance)
  | 'R' => (Right, distance)
  | _ => raise(UnknownDirection)
  };
};

let lineToDirections = str => {
  str |> String.split_on_char(',') |> List.map(stringToDirection);
};

let directionsToCoordMap = dirs => {
  let coordMap = ref(CoordMap.empty);
  let x = ref(0);
  let y = ref(0);
  let steps = ref(0);
  let coordToString = () => string_of_int(x^) ++ "," ++ string_of_int(y^);
  let addCoord = () =>
    coordMap := CoordMap.add(coordToString(), steps^, coordMap^);
  dirs
  |> List.iter(((direction, distance)) => {
       for (_ in 1 to distance) {
         steps := steps^ + 1;
         switch (direction) {
         | Up => y := y^ + 1
         | Down => y := y^ - 1
         | Left => x := x^ - 1
         | Right => x := x^ + 1
         };
         addCoord();
       }
     });
  coordMap^;
};

let wires =
  "./solutions/day-03/input.txt"
  |> Core.In_channel.read_lines
  |> List.map(lineToDirections);

let firstCoordMap = wires |> List.hd |> directionsToCoordMap;
let secondCoordMap = List.nth(wires, 1) |> directionsToCoordMap;

firstCoordMap
|> CoordMap.fold(
     (coord, firstSteps, smallestSteps) => {
       let secondStepsOpt = secondCoordMap |> CoordMap.find_opt(coord);
       switch (secondStepsOpt) {
       | Some(secondSteps) =>
         let steps = firstSteps + secondSteps;
         steps < smallestSteps ? steps : smallestSteps;
       | None => smallestSteps
       };
     },
     _,
     Int.max_int,
   )
|> Printf.printf("Steps: %d\n");