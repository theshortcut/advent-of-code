open Core;

let file = In_channel.read_lines("./solutions/day-01/input.txt");

let calculateFuel = mass => int_of_float(mass /. 3.) - 2;

let sumFuel = (fuelAcc, line) =>
  fuelAcc + (line |> float_of_string |> calculateFuel);

let totalFuel = file |> List.fold(~init=0, ~f=sumFuel);

print_endline("Sum of fuel:");
print_endline(totalFuel |> string_of_int);