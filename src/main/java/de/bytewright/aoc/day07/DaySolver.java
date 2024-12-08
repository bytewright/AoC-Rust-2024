package de.bytewright.aoc.day07;

import de.bytewright.aoc.day07.data.EquationData;
import de.bytewright.aoc.day07.data.EquationEval;
import org.apache.commons.lang3.StringUtils;
import org.slf4j.Logger;
import org.springframework.stereotype.Component;

import java.util.*;
import java.util.concurrent.ExecutionException;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.Future;

@Component
public class DaySolver {
    private static final Logger LOGGER = org.slf4j.LoggerFactory.getLogger(DaySolver.class);

    public String computeResult(List<String> inputData) {
        ExecutorService executorService = Executors.newFixedThreadPool(8);
        List<EquationDataWorker> dataList = inputData.stream()
                                                .map(this::parseEquationData)
                                                .flatMap(Optional::stream)
                                                .sorted(Comparator.comparing(equationData -> equationData.args()
                                                                                                 .size()))
                                                .map(equationData -> new EquationDataWorker(equationData, executorService))
                                                .toList();
        LOGGER.info("All {} task workers created, submitting...", dataList.size());
        List<Future<EquationEval>> results = dataList.stream()
                                                 .map(executorService::submit)
                                                 .toList();
        LOGGER.info("All {} task workers submitted, starting result collection...", dataList.size());
        String result = collectResults(results);
        executorService.shutdown();
        return result;
    }

    private String collectResults(List<Future<EquationEval>> results) {
        int validCounter = 0;
        int resultCounter = 0;
        long validSum = 0;
        for (Future<EquationEval> result : results) {
            try {
                EquationEval equationEval = result.get();
                resultCounter++;
                if (equationEval.valid()) {
                    LOGGER.info("{}/{} has a valid result and contains {} nodes: {}", resultCounter, results.size(), equationEval.nodeCount(), equationEval.solution());
                    validCounter++;
                    validSum += equationEval.result();
                }
            } catch (InterruptedException | ExecutionException e) {
                throw new RuntimeException(e);
            }
        }
        LOGGER.info("Found {} valid results, total sum: {}", validCounter, validSum);
        return "%d".formatted(validSum);
    }

    private Optional<EquationData> parseEquationData(String input) {
        try {
            String result = input.split(":")[0];
            List<Long> args = Arrays.stream(input.split(":")[1].split(" "))
                                  .map(StringUtils::trimToNull)
                                  .filter(Objects::nonNull)
                                  .map(Long::parseLong)
                                  .toList();
            return Optional.of(new EquationData(Long.parseLong(result), args));
        } catch (Exception e) {
            LOGGER.warn("Unparseable: {}", input);
            return Optional.empty();
        }
    }
}
