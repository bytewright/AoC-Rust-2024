package de.bytewright.aoc.day07;

import de.bytewright.aoc.day07.data.EquationData;
import de.bytewright.aoc.day07.data.EquationEval;
import de.bytewright.aoc.day07.data.Operator;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.time.Duration;
import java.time.Instant;
import java.util.*;
import java.util.concurrent.Callable;
import java.util.concurrent.ExecutorService;
import java.util.stream.Collectors;

public class EquationDataWorker implements Callable<EquationEval> {
    private static final Logger LOGGER = LoggerFactory.getLogger(EquationDataWorker.class);
    private static final List<Operator> OPERATORS = List.of(Operator.ADD, Operator.MULTIPLY, Operator.CONCAT);
    private final EquationData workPackage;
    private final ExecutorService executorService;

    public EquationDataWorker(EquationData workPackage, ExecutorService executorService) {
        this.workPackage = workPackage;
        this.executorService = executorService;
    }

    private static TreeNode2 generateTrees(ExecutorService executorService, long result, Queue<Long> args) {
        long nextArg = args.poll();
        return doGenerateTree(executorService, result, null, null, nextArg, null, nextArg, args);
    }

    private static TreeNode2 doGenerateTree(ExecutorService executorService, long result, TreeNode2 root, TreeNode2 parent, long currentArg, Operator currentOp, long eval, Queue<Long> args) {
        if (args.isEmpty()) {
            TreeNode2 leaf = new TreeNode2(parent, currentArg, eval, currentOp);
            root.addLeafNode(leaf);
            root.incNodeCount();
            return leaf;
        }
        Long nextArg = args.poll();
        TreeNode2 node = new TreeNode2(parent, currentArg, eval, currentOp);
        List<TreeNode2> childNodes = new ArrayList<>(OPERATORS.size());
        for (Operator operator : OPERATORS) {
            long thisEval = operator.eval(eval, nextArg);
            if (thisEval <= result) {
                childNodes.add(doGenerateTree(executorService, result, root == null ? node : root, node, nextArg, operator, thisEval, new LinkedList<>(args)));
            }
        }
        node.addChildren(childNodes);
        if (root != null) root.incNodeCount();
        return node;
    }

    @Override
    public EquationEval call() {
        Instant startTime = Instant.now();
        try {
            Optional<List<TreeNode2>> solution = findSolution();
            Duration duration = Duration.between(startTime, Instant.now());
            if (solution.isPresent()) {
                List<TreeNode2> nodes = solution.get();
                String equation = workPackage.result() + " = " + nodes.stream()
                                                                     .map(TreeNode2::asStringEq)
                                                                     .collect(Collectors.joining(""))
                                                                     .trim();
                return new EquationEval(true, workPackage.result(), equation, nodes.get(0)
                                                                                  .getNodeCount(), duration);
            }
        } catch (Exception e) {
            LOGGER.error("Error processing equation", e);
        }
        return new EquationEval(false, -1, null, 0, Duration.between(startTime, Instant.now()));
    }

    private Optional<List<TreeNode2>> findSolution() {
        Queue<Long> argsQueue = new LinkedList<>(workPackage.args());
        TreeNode2 root = generateTrees(executorService, workPackage.result(), argsQueue);

        for (TreeNode2 leafNode : root.getLeafNodes()) {
            if (workPackage.result() == leafNode.getEvalToThisNode()) {
                return Optional.of(leafNode.getPath());
            }
        }
        return Optional.empty();
    }

}
