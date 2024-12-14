# NopeLang

*NopeLang* is a high-level language built for the *NopeVM*. It aims to be simple and approachable, while still exposing the full power of the underlying *NopeVM* to the developer.

*NopeLang* scripts consist of two main components: an *entrypoint*, and it's contained *expressions*.

## The Entrypoint

The entrypoint marks the beginning of execution in a *NopeLang* script. For simplicity, a script can only define one entrypoint. Entrypoints are defined by the keyword `entrypoint`.

A simple script that does nothing can be defined as follows:

```nopelang
entrypoint {}
```

Note that whitespace is not significant in *NopeLang*, other than for token separation. The following script is equivalent to the script above:

```nopelang
  entrypoint{      

    }
```

## Expressions

The body of an entrypoint contains a sequence of *expressions*.

Valid expressions:

| expression | description  |
| ---------- | ------------ |
| `noop`     | no operation |

A simple script that does nothing can be defined as follows:

```nopelang
entrypoint {
    noop
}
```

Multiple expressions are separated by whitespace:

```nopelang
entrypoint {
    noop
    noop
}
```
