open Core;

let file = In_channel.read_lines("./solutions/day-01/input.txt");

// first star
// let calculateFuel = mass => int_of_float(mass /. 3.) - 2;

// second star
let rec calculateFuel = (fuelAcc, mass) => {
  let fuel = int_of_float(float_of_int(mass) /. 3.) - 2;
  fuel <= 0 ? fuelAcc : calculateFuel(fuel + fuelAcc, fuel);
};

let sumFuel = (fuelAcc, line) =>
  fuelAcc + (line |> int_of_string |> calculateFuel(0));

let totalFuel = file |> List.fold(~init=0, ~f=sumFuel);

print_endline("Sum of fuel:");
print_endline(totalFuel |> string_of_int);