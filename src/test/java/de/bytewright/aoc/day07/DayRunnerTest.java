package de.bytewright.aoc.day07;

import org.junit.jupiter.api.Test;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.context.SpringBootTest;

//@SpringBootTest
class DayRunnerTest {
    private DaySolver daySolver = new DaySolver();

    @Test
    void testRunDaySolver() {
        DayRunner dayRunner = new DayRunner(daySolver);
        dayRunner.doRun("examples");
    }
}
