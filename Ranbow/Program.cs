using Microsoft.AspNetCore.Hosting;
using Microsoft.Extensions.Hosting;

using RanbowBack.Config;
using RanbowBack.Repositories.Base;

namespace Ranbow
{
	public class Program
	{
		public static void Main(string[] args)
		{
			BaseRepository repo = new(Configuration.Instance.Init().ConnectionString);
			if (repo.Check())
				repo.Dispose();
				CreateHostBuilder(args).Build().Run();
		}

		public static IHostBuilder CreateHostBuilder(string[] args) =>
			Host.CreateDefaultBuilder(args)
				.ConfigureWebHostDefaults(webBuilder =>
				{
					webBuilder.UseStartup<Startup>();
				});
	}
}
