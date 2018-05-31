using System.Collections.Generic;
using Oak.OverUnder.Domain.GamePlay.Models;

namespace Oak.OverUnder.Domain.GamePlay
{
    public class GameRound
    {
        public Card HiddenCard { get; set; }
        public Card VisibleCard { get; set; }
        public GameRoundState State { get; set; }

        public static GameRound Create(List<Card> cards)
        {
            var round = new GameRound();
            round.State = GameRoundState.InProgress;
            round.HiddenCard = cards[0];
            round.VisibleCard = cards[1];
            return round;
        }

        public void ChooseUnder()
        {
            if (HiddenCard.Number < VisibleCard.Number)
            {
                State = GameRoundState.PlayerWins;
            }
            else
            {
                State = GameRoundState.OponentWins;
            }
        }

        public void ChooseOver()
        {
            if (HiddenCard.Number > VisibleCard.Number)
            {
                State = GameRoundState.PlayerWins;
            }
            else
            {
                State = GameRoundState.OponentWins;
            }
        }
    }

    public enum GameRoundState
    {
        None = 0,

        InProgress = 1,

        PlayerWins = 2,

        OponentWins = 3
    }
}
