let validCount = ref(0);

let hasAdjacentDigits = indexedDigits =>
  indexedDigits
  |> List.find_opt(((i, digit)) => {
       switch (List.nth_opt(indexedDigits, i + 1)) {
       | Some((_, nextDigit)) => nextDigit == digit
       | None => false
       }
     })
  |> Option.is_some;

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

  hasAdjacentDigits(indexedDigits) && neverDecreases(indexedDigits);
};

for (candidate in 235741 to 706948) {
  if (candidate |> string_of_int |> validate) {
    validCount := validCount^ + 1;
  };
};

Printf.printf("Valid passwords: %d", validCount^);