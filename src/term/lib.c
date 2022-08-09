#include <termios.h>

static struct termios term_old;

int termios_icanon_echo()
{
    struct termios term_new;

    if (tcgetattr(0, &term_old))
    {
        return -1;
    }

    term_new = term_old;
    term_new.c_lflag &= ~ICANON;
    term_new.c_lflag |= ECHO;

    if (tcsetattr(0, TCSANOW, &term_new))
    {
        return -2;
    }

    return 0;
}

int termios_revert()
{
    if (tcsetattr(0, TCSANOW, &term_old))
    {
        return -2;
    }

    return 0;
}
