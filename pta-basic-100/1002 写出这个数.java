import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        Scanner in = new Scanner(System.in);
        String str = in.next();
        char[] ca = str.toCharArray();
        int sum = 0;
        for(char c: ca) {
            sum += c - '0';
        }
        String sumStr = String.valueOf(sum);
        String[] pinyin = {
                "ling","yi","er","san","si","wu",
                "liu","qi","ba","jiu"
        };
        for(int i = 0; i < sumStr.length(); i++) {
            int digit = sumStr.charAt(i) - '0';
            if(i == sumStr.length() - 1)
                System.out.print(pinyin[digit]);
            else
                System.out.print(pinyin[digit] + " ");
        }
    }
}