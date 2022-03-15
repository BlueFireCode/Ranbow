using Microsoft.AspNetCore.Hosting;
using Microsoft.Extensions.Hosting;
using RanbowBack.Config;
using RanbowBack.Repositories.Base;
using System;
using System.IO;

namespace Ranbow
{
    public class Program
    {
        public static void Main(string[] args)
        {
            //Load settings.env
            if (File.Exists("settings.env"))
            {
                var lines = File.ReadAllLines("settings.env");
                foreach (var line in lines)
                {
                    Environment.SetEnvironmentVariable(line[..line.IndexOf('=')], line[(line.IndexOf('=') + 1)..]);
                }
            }

            BaseRepository repo = new(Configuration.Instance.Init().ConnectionString);
            if (repo.Check())
            {
                repo.Dispose();
                CreateHostBuilder(args).Build().Run();
            }
        }

        public static IHostBuilder CreateHostBuilder(string[] args) =>
            Host.CreateDefaultBuilder(args)
                .ConfigureWebHostDefaults(webBuilder =>
                {
                    webBuilder.UseStartup<Startup>();
                });
    }
}
