import gin


@gin.configurable
def inner(d=0, e=0):
    return d + e


@gin.configurable
def build(a=0, b=0, c=0, inner=None):
    s = inner()
    print(a + b + c + s)


@gin.configurable
def build_other(inner=None):
    print(inner())


if __name__ == "__main__":
    gin.parse_config_file("main.gin")
    build()
    build_other()
