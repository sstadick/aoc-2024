from tensor import Tensor
from utils import Span

from ExtraMojo.bench.bench import Bench


fn count_items(borrowed items: List[UInt8]) -> Int:
    return len(items)


fn count_items_span(items: Span[UInt8]) -> Int:
    return len(items)


fn count_items_list(borrowed items: List[UInt8], offset: Int) -> Int:
    return len(items) - offset


fn count_items_tensor(borrowed items: Tensor[DType.uint8], offset: Int) -> Int:
    """Mock function that would work on the tensor based on the offset instead of a slice.
    """
    return items.num_elements() - offset


fn main() raises:
    var item_list = List[UInt8]()
    for i in range(10000):
        item_list.append(i)

    fn test_passing_slice() raises:
        var item_span = Span(item_list)

        var sum = 0
        for i in range(10000):
            sum += count_items_span(item_span[i:])

    fn test_passing_list() raises:
        var sum = 0
        for i in range(10000):
            sum += count_items_list(item_list, i)

    var item_tensor = Tensor(item_list)

    fn test_passing_tensor() raises:
        var sum = 0
        for i in range(10000):
            sum += count_items_tensor(item_tensor, i)

    var slice_bench = Bench(
        name="test_passing_slice",
        max_iterations=10000,
        warmup_iterations=100,
        func=test_passing_slice,
    )
    slice_bench.benchmark()

    var list_bench = Bench(
        name="test_passing_list",
        max_iterations=10000,
        warmup_iterations=100,
        func=test_passing_list,
    )
    list_bench.benchmark()

    var tensor_bench = Bench(
        name="test_passing_tensor",
        max_iterations=10000,
        warmup_iterations=100,
        func=test_passing_tensor,
    )
    tensor_bench.benchmark()
