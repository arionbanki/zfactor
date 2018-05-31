using System;
using System.Collections.Generic;
using System.Text;

namespace Oak.OverUnder.Domain.GamePlay.Models
{
    public static class Enums
    {
        public enum Suit { H = 0, D, C, S }
        public enum Number { _JOKER = 0, _2, _3, _4, _5, _6, _7, _8, _9, _10, _J, _Q, _K, _A }
        public enum Back { Red = 0, Blue }
    }
}
