import test, {ExecutionContext} from "ava";

import {paginate} from "../../src/ts/utils";

const paginationMacro = (
  t: ExecutionContext,
  current: number,
  total: number,
  expected: Array<number | string>,
): void => {
  t.deepEqual(paginate(current, total), expected);
};

paginationMacro.title = (
  _title: string,
  current: number,
  total: number,
  expected: Array<number | string>,
): string => `paginate(${current},${total}) => [${expected}]`.trim();

test(paginationMacro, 0, 0, []);
test(paginationMacro, 0, 7, [0, 1, 2, 3, 4, 5, 6]);
test(paginationMacro, 0, 10, [0, 1, 2, 3, 4, "…", 9]);
test(paginationMacro, 3, 10, [0, 1, 2, 3, 4, "…", 9]);
test(paginationMacro, 4, 10, [0, "…", 3, 4, 5, "…", 9]);
test(paginationMacro, 10, 10, [0, "…", 5, 6, 7, 8, 9]);
test(paginationMacro, 7, 15, [0, "…", 6, 7, 8, "…", 14]);
test(paginationMacro, 8, 15, [0, "…", 7, 8, 9, "…", 14]);
