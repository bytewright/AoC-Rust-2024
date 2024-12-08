package de.bytewright.aoc.day07.data;

import java.time.Duration;

public record EquationEval(boolean valid, long result, String solution, long nodeCount, Duration processingTime) {
}
