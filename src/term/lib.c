/**
 * @file
 * @authors Julian Merkle
 * @copyright 2022 Richard Wolf GmbH
 */

#include <termios.h>

int tcsetattr_icanon_echo()
{
    struct termios term;

    if (tcgetattr(0, &term))
    {
        return -1;
    }

    term.c_lflag &= ~ICANON;
    term.c_lflag |= ECHO;

    if (tcsetattr(0, TCSANOW, &term))
    {
        return -2;
    }

    return 0;
}
