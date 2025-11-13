main = || (
  fd = 1
  msg = string_literal("Hello World\n")
  len = 12
  _ = write(fd, msg, len)
  exit(0)
)
