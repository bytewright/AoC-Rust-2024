package de.bytewright.aoc.day07;

import lombok.AllArgsConstructor;
import lombok.NoArgsConstructor;
import org.slf4j.Logger;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.shell.standard.ShellComponent;
import org.springframework.shell.standard.ShellMethod;
import org.springframework.util.StopWatch;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;
import java.util.concurrent.TimeUnit;

@ShellComponent
@NoArgsConstructor
@AllArgsConstructor
public class DayRunner {
    private static final Logger LOGGER = org.slf4j.LoggerFactory.getLogger(DayRunner.class);
    private static final String DAY = "07";
    @Autowired
    private DaySolver daySolver;

    @ShellMethod(key = DAY)
    public String runDaySolver() {
        return doRun("inputs");

    }

    String doRun(String dataDir) {
        StopWatch stopWatch = new StopWatch("Day %s timings".formatted(DAY));
        stopWatch.start("Data Load");
        List<String> inputData = loadData(dataDir, DAY);
        stopWatch.stop();
        stopWatch.start("Result computation");
        String result = daySolver.computeResult(inputData);
        stopWatch.stop();
        LOGGER.info("Finished DAY {} computations!\nResult: {}\n{}", DAY, result, stopWatch.prettyPrint(TimeUnit.MILLISECONDS));
        return result;
    }

    private List<String> loadData(String dataDir, String day) {
        Path dataPath = Path.of("data", dataDir, "%s.txt".formatted(day));
        try {
            return Files.readAllLines(dataPath);
        } catch (IOException e) {
            LOGGER.error("Failed to load data from {}", dataPath.toAbsolutePath());
            throw new RuntimeException(e);
        }
    }
}
