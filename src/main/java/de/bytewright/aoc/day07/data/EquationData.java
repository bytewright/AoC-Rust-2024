package de.bytewright.aoc.day07.data;

import java.util.List;
import java.util.stream.Collectors;

public record EquationData(long result, List<Long> args) {
    public String rawArgs() {
        return args.stream()
                   .map(Object::toString)
                   .collect(Collectors.joining(" "));
    }
}
