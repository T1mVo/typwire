#import "utils.typ": is-debug-assert

#let type-name = "datetime"

#let new(
  year: none,
  month: none,
  day: none,
  hour: none,
  minute: none,
  second: none,
) = {
  assert(
    type(year) == std.int or year == none,
    message: "datetime.new: year must be type of int or none",
  )
  assert(
    type(month) == std.int or month == none,
    message: "datetime.new: month must be type of int or none",
  )
  assert(
    type(day) == std.int or day == none,
    message: "datetime.new: day must be type of int or none",
  )
  assert(
    type(hour) == std.int or hour == none,
    message: "datetime.new: hour must be type of int or none",
  )
  assert(
    type(minute) == std.int or minute == none,
    message: "datetime.new: minute must be type of int or none",
  )
  assert(
    type(second) == std.int or second == none,
    message: "datetime.new: second must be type of int or none",
  )

  (
    typwire-type: type-name,
    year: year,
    month: month,
    day: day,
    hour: hour,
    minute: minute,
    second: second,
  )
}

/// Encode a datetime into a CBOR-compatible dictionary.
///
/// - datetime (datetime): A datetime.
/// -> dictionary
#let encode(datetime) = {
  if is-debug-assert {
    assert(
      type(datetime) == std.datetime,
      message: "datetime.encode: datetime must be type of datetime",
    )
  }

  new(
    year: datetime.year(),
    month: datetime.month(),
    day: datetime.day(),
    hour: datetime.hour(),
    minute: datetime.minute(),
    second: datetime.second(),
  )
}

/// Decode a CBOR-compatible dictionary into a datetime.
///
/// - value (dictionary): A dictionary representing the `encode`d value.
/// -> datetime
#let decode(value) = {
  if is-debug-assert {
    assert(
      value.typwire-type == type-name,
      message: "datetime.decode: value must be a tagged with type "
        + type-name
        + ". Found "
        + value.typwire-type,
    )
    assert(
      value.fields().all(field => type(field) == std.int or field == none),
      message: "datetime.decode: all fields must be of type int or none",
    )
  }

  let fields = (:)

  for field in ("year", "month", "day", "hour", "minute", "second") {
    if value.at(field) != none {
      fields.insert(field, value.at(field))
    }
  }

  datetime(..fields)
}
