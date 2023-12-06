#define CL_HPP_ENABLE_EXCEPTIONS
#include <CL/opencl.hpp>

#include <algorithm>
#include <cstdlib>
#include <iostream>
#include <iterator>
#include <limits>
#include <random>
#include <vector>

typedef struct __attribute__((packed)) {
    int64_t destination;
    int64_t source;
    int64_t length;
} Range;

struct RangeMap {
    std::vector<Range> map;
};

typedef struct __attribute__((packed)) {
    int64_t first_index;
    int64_t length;
} RawRangeMap;

std::istream& operator>>(std::istream& is, Range& range) {
    int64_t x;
    int64_t y;
    int64_t z;
    is >> x >> y >> z;
    range.destination = x;
    range.source = y;
    range.length = z;

    return is;
}

std::ostream& operator<<(std::ostream& os, Range const& range) {
    return os << "Range{ " << range.length << " " << range.source << " -> " << range.destination << " }";
}

std::istream& operator>>(std::istream& is, RangeMap& map) {
    is.clear();

    Range r;
    map.map = {};

    is >> r;
    while(!is.fail()) {
        map.map.push_back(r);
        is >> r;
    }

    return is;
}

std::string source = R"(
#pragma OPENCL EXTENSION cl_khr_int64_base_atomics : enable
#pragma OPENCL EXTENSION cl_khr_int64_extended_atomics : enable

typedef struct __attribute__((packed)) r {
    long destination;
    long source;
    long length;
} Range;

typedef struct __attribute__((packed)) rrm {
    long first_index;
    long length;
} RawRangeMap;

__kernel void evaluate_seed(const long first_seed, __global const Range* ranges, __global const RawRangeMap* maps, const int maps_count, __global atomic_long* result) {
    int id = get_global_id(0);

    if(id == 0) {
        atomic_init(result, ~(1 << 63));
    }

    long value = first_seed + id;

    for(int i = 0; i < maps_count; i++) {
        RawRangeMap map = maps[i];

        for(int j = 0; j < map.length; j++) {
            Range range = ranges[map.first_index + j];

            if(range.source <= value && range.source + range.length >= value) {
                value += range.destination - range.source;
                break;
            }
        }
    }

    atom_min(result, value);
}
)";

using kernel_type = cl::compatibility::make_kernel<int, cl::Buffer, cl::Buffer, int, cl::Buffer>;

int64_t calculate_partial_result(cl::Context& context, cl::CommandQueue& queue, kernel_type& kernel, cl::Buffer ranges_buf, cl::Buffer maps_buf, int maps_count, int first_seed, int cnt) {
    cl::Buffer out_buf(context, CL_MEM_WRITE_ONLY, sizeof(int64_t));

    kernel(cl::EnqueueArgs(queue, cl::NDRange(cnt), cl::NDRange(500)), first_seed, ranges_buf, maps_buf, maps_count, out_buf);
    int64_t result;
    cl::copy(queue, out_buf, &result, &result + 1);

    return result;
}

int main() {
    std::string tmp1, tmp2;
    std::cin >> tmp1;

    std::vector<int64_t> seeds{};
    std::vector<RangeMap> maps{};

    int64_t tmp;
    std::cin >> tmp;

    while(!std::cin.fail()) {
        seeds.push_back(tmp);
        std::cin >> tmp;
    }

    std::cin.clear();

    for(int i = 0; i < 7; i++) {
        std::cin.clear();
        std::cin >> tmp1 >> tmp2;
        RangeMap map;
        std::cin >> map;
        maps.push_back(map);
    }

    std::vector<Range> all_ranges{};
    std::vector<RawRangeMap> raw_maps{};

    for(RangeMap const& m : maps) {
        RawRangeMap rm {
            .first_index = (int64_t)all_ranges.size(),
            .length = (int64_t)m.map.size()
        };

        for(Range r : m.map) {
            all_ranges.push_back(r);
        }

        raw_maps.push_back(rm);
    }

    cl::Context context(CL_DEVICE_TYPE_DEFAULT);
    cl::CommandQueue queue(context);
    cl::Program program(context, source, true);

    cl::Buffer ranges_buf(context, all_ranges.cbegin(), all_ranges.cend(), true);
    cl::Buffer maps_buf(context, raw_maps.cbegin(), raw_maps.cend(), true);

    kernel_type kernel(program, "evaluate_seed");

    int64_t result = std::numeric_limits<int64_t>::max();

    for(int i = 0; i < seeds.size(); i += 2) {
        int first_seed = seeds[i];
        int cnt = seeds[i + 1];
        result = std::min(
            result,
            calculate_partial_result(context, queue, kernel, ranges_buf, maps_buf, raw_maps.size(), first_seed, cnt)
        );
    }

    std::cout << result << std::endl;

    return 0;
}