using System;
using System.Collections.Generic;
using System.Text;

namespace Oak.OverUnder.Domain.GamePlay.Models
{
    public class Card
    {
        public Enums.Suit Suit { get; set; }
        public Enums.Number Number { get; set; }
        public Enums.Back Back { get; set; }

        public int Position { get; set; }

        public Card()
        {

        }
    }

}
