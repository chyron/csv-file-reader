interface Options {
  seperator?: string
  header?: boolean
}

export function parse(path: string, options?: Options): { [k: string]: string }[] | string[][]
