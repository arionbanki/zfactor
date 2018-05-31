using Oak.OverUnder.Domain;

namespace Oak.OverUnder.Tests
{
    public class GameBuilder
    {
        public Game Create()
        {
            var game = new Game();
            game.NewCards += delegate (Game obj) { obj.ChooseOver(); };
            return game;
        }

        public Game CreateWithOponentWithMorePoints()
        {
            var game = Create();
            game.TotalRounds = 26;
            game.Player.Points = 12;
            game.Oponent.Points = 14;
            return game;
        }

        public Game CreateWithPlayerWithMorePoints()
        {
            var game = Create();
            game.TotalRounds = 26;
            game.Player.Points = 14;
            game.Oponent.Points = 12;
            return game;
        }
    }
}
