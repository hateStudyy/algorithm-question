import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);
        int n = scanner.nextInt();
        scanner.nextLine(); // 消耗掉nextInt后的换行符

        for (int i = 0; i < n; i++) {
            String s = scanner.nextLine();
            if (isValid(s)) {
                System.out.println("YES");
            } else {
                System.out.println("NO");
            }
        }
        scanner.close();
    }

    private static boolean isValid(String s) {
        int pCount = 0, tCount = 0;
        int pIndex = -1, tIndex = -1;

        // 检查是否只包含P、A、T，以及统计P和T的数量和位置
        for (int i = 0; i < s.length(); i++) {
            char c = s.charAt(i);
            if (c == 'P') {
                pCount++;
                pIndex = i;
            } else if (c == 'T') {
                tCount++;
                tIndex = i;
            } else if (c != 'A') {
                return false; // 包含其他字符
            }
        }

        // 检查P和T的数量是否正确
        if (pCount != 1 || tCount != 1) {
            return false;
        }

        // 检查P是否在T之前
        if (pIndex >= tIndex) {
            return false;
        }

        // 计算a、b、c
        int a = pIndex;
        int b = tIndex - pIndex - 1;
        int c = s.length() - tIndex - 1;

        // 检查b是否至少为1，以及a*b是否等于c
        return b >= 1 && a * b == c;
    }
}