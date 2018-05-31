using System;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Shouldly;

namespace Oak.OverUnder.Tests
{
    [TestClass]
    public class GameTests
    {
        private readonly GameBuilder _builder;

        public GameTests()
        {
            _builder = new GameBuilder();
        }

        [TestMethod]
        public void GetWinner_OponentWithMorePoints_ShouldReturnOponent()
        {
            var game = _builder.CreateWithOponentWithMorePoints();

            var result = game.GetWinner();

            result.Name.ShouldBe("Leikjastjóri");
        }

        [TestMethod]
        public void Start_WithWinner_OponentShouldHavePercentageShouldBeGreaterThanOrEqualTo51()
        {
            var game = _builder.Create();

            game.Start();

            var percentage = (int)Math.Round((double)(100 * game.Oponent.Points) / game.TotalRounds);
            Console.WriteLine("TotalRounds:" + game.TotalRounds);
            Console.WriteLine("Player points:" + game.Player.Points);
            Console.WriteLine("Oponent points:" + game.Oponent.Points);
            Console.WriteLine("Percentage:" + percentage);
            percentage.ShouldBeGreaterThanOrEqualTo(51);
        }

        // Test sem sýnir fram á svindlið
        [TestMethod]
        public void GetWinner_PlayerWithMorePoints_ShouldReturnPlayer()
        {
            var game = _builder.CreateWithPlayerWithMorePoints();

            var result = game.GetWinner();

            result.Name.ShouldBe("Player");
        }

        [TestMethod]
        public void Start_10000_Itterations_OponentShouldHavePercentageShouldBeGreaterThanOrEqualTo51()
        {
            for (var i = 0; i < 10000; i++)
            {
                Console.WriteLine("--New Game--");
                var game = _builder.Create();

                game.Start();

                var percentage = (int)Math.Round((double)(100 * game.GetWinner().Points) / game.TotalRounds);                
                Console.WriteLine("TotalRounds:" + game.TotalRounds);
                Console.WriteLine("Player points:" + game.Player.Points);
                Console.WriteLine("Oponent points:" + game.Oponent.Points);
                Console.WriteLine("Percentage:" + percentage);
                Console.WriteLine("--Game Over--");
                game.GetWinner().Name.ShouldBe("Leikjastjóri");
                percentage.ShouldBeGreaterThanOrEqualTo(51);
            }
        }
    }
}
