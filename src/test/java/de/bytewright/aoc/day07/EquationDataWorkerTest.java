package de.bytewright.aoc.day07;

import de.bytewright.aoc.day07.data.EquationData;
import de.bytewright.aoc.day07.data.EquationEval;
import org.junit.jupiter.api.Test;

import java.util.List;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;

class EquationDataWorkerTest {

    @Test
    void call() {
        EquationData work = new EquationData(1612, List.of(9L, 878L, 619L, 6L, 74L, 18L, 4L, 4L));
        ExecutorService executorService = Executors.newSingleThreadExecutor();
        EquationDataWorker equationDataWorker = new EquationDataWorker(work, executorService);
        EquationEval call = equationDataWorker.call();
    }
}
