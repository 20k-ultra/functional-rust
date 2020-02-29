# Leap

Given a year, report if it is a leap year.

The tricky thing here is that a leap year in the Gregorian calendar occurs:

```text
on every year that is evenly divisible by 4
  except every year that is evenly divisible by 100
    unless the year is also evenly divisible by 400
```

For example, 1997 is not a leap year, but 1996 is.  1900 is not a leap
year, but 2000 is.

## Testing

Execute the tests with:

```bash
$ cargo test
```

## Source

JavaRanch Cattle Drive, exercise 3 [http://www.javaranch.com/leap.jsp](http://www.javaranch.com/leap.jsp)

## Credit

This challenge was taken from the Rust [track](https://exercism.io/my/tracks/rust) on [exercism.io](https://exercism.io).
