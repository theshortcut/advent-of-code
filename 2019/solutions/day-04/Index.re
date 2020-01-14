let validCount = ref(0);

let hasOnlyTwoAdjacentDigits = indexedDigits =>
  indexedDigits
  |> List.fold_left(
       (valid, (i, digit)) =>
         if (i > 0) {
           let prevDigitOpt = List.nth_opt(indexedDigits, i - 1);
           let nextDigitOpt = List.nth_opt(indexedDigits, i + 1);
           let lookaheadOpt = List.nth_opt(indexedDigits, i + 2);
           switch (prevDigitOpt, nextDigitOpt, lookaheadOpt) {
           | (Some((_, prev)), Some((_, next)), Some((_, lookAhead))) =>
             if (prev == digit && next != digit && i == 1) {
               true;
             } else if (prev != digit && next == digit && lookAhead != digit) {
               true;
             } else {
               valid;
             }
           | (Some((_, prev)), Some((_, next)), None) =>
             prev != digit && next == digit ? true : valid
           | _ => valid
           };
         } else {
           valid;
         },
       false,
     );

let neverDecreases = indexedDigits =>
  indexedDigits
  |> List.fold_left(
       (valid, (i, digit)) => {
         switch (List.nth_opt(indexedDigits, i + 1)) {
         | Some((_, nextDigit)) => valid && digit <= nextDigit
         | None => valid
         }
       },
       true,
     );

let validate = password => {
  let indexedDigits =
    password
    |> String.to_seq
    |> List.of_seq
    |> List.map(int_of_char)
    |> List.mapi((i, d) => (i, d));

  hasOnlyTwoAdjacentDigits(indexedDigits) && neverDecreases(indexedDigits);
};

for (candidate in 235741 to 706948) {
  if (candidate |> string_of_int |> validate) {
    validCount := validCount^ + 1;
  };
};

Printf.printf("Valid passwords: %d", validCount^);