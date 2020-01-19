let foo = () => print_endline("Util");

module Int = {
  let digits = int => {
    let rec digit = (acc, d) =>
      if (d < 10) {
        [d, ...acc];
      } else {
        digit([d mod 10, ...acc], d / 10);
      };
    digit([], int);
  };
};