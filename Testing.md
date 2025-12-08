# Testing Methodology

This library is tested against the Java library KosherJava.

Dates from the years 1870 to 2070 are tested.

We allows for 50ms difference between the Java and Rust implementations.

For other calculations, such as solar elevation and azimuth, we allow for a .02  difference.

There are many instances where the Java library would return Nan or Long.MIN_VALUE to indicate null or invalid values.
We use the `Option<T>` type in Rust to handle these cases and return None instead.

This library also handles invalid timezones differently than Java. Creating a DateTime in a rust with an invalid timezone will return None, whereas Java will return a GMT timezone. This is acounted for in testing.

In all places where java would throw an exception, we return None instead. We never throw under any circumstances

There are some timezones which are not supported by Java. These are not tested.

Java's datetime library are more flexible in how they deal with DST transitions, while we are very strict. Any computation that can result in an ambiguous time, or a time which is invalid for the given timezone, will return None. Becuase of this we when comparing testing options, we allow the rust one to be None, and the java one to be Some. We limit this to .05% of all iterations to ensure we arent missing any valid bugs in the software

Calling one of the following methods with a negative hours parameter will return None:

- `get_half_day_based_zman_from_times`
- `get_shaah_zmanis_based_zman_from_times`
