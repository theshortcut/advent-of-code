type direction =
  | Up
  | Down
  | Left
  | Right;

exception UnknownDirection;

type coord = (int, int);

module CoordSet =
  Set.Make({
    type t = coord;
    let compare = (a, b) =>
      if (a == b) {
        0;
      } else if (a > b) {
        1;
      } else {
        (-1);
      };
  });

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
  let coordSet = ref(CoordSet.empty);
  let x = ref(0);
  let y = ref(0);
  let addCoord = () => coordSet := CoordSet.add((x^, y^), coordSet^);
  dirs
  |> List.iter(((direction, distance)) => {
       switch (direction) {
       | Up =>
         for (_ in 1 to distance) {
           y := y^ + 1;
           addCoord();
         }
       | Down =>
         for (_ in 1 to distance) {
           y := y^ - 1;
           addCoord();
         }
       | Left =>
         for (_ in 1 to distance) {
           x := x^ - 1;
           addCoord();
         }
       | Right =>
         for (_ in 1 to distance) {
           x := x^ + 1;
           addCoord();
         }
       }
     });
  coordSet^;
};

let wires =
  "./solutions/day-03/input.txt"
  |> Core.In_channel.read_lines
  |> List.map(lineToDirections);

let firstCoordSet = wires |> List.hd |> directionsToCoordMap;
let secondCoordSet = List.nth(wires, 1) |> directionsToCoordMap;

CoordSet.inter(firstCoordSet, secondCoordSet)
|> CoordSet.fold(
     ((x, y), smallestDistance) => {
       let distance = abs(x) + abs(y);
       distance < smallestDistance ? distance : smallestDistance;
     },
     _,
     Int.max_int,
   )
|> Printf.printf("Distance: %d");