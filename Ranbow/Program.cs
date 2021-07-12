using Microsoft.AspNetCore.Hosting;
using Microsoft.Extensions.Hosting;
using System;
using System.Data.SqlClient;

using RanbowBack.Config;

namespace Ranbow
{
	public class Program
	{
		public static void Main(string[] args)
		{
			if (ValidateConnection(Configuration.Instance.Init().ConnectionString))
				CreateHostBuilder(args).Build().Run();
		}

		public static IHostBuilder CreateHostBuilder(string[] args) =>
			Host.CreateDefaultBuilder(args)
				.ConfigureWebHostDefaults(webBuilder =>
				{
					webBuilder.UseStartup<Startup>();
				});

		private static bool ValidateConnection(string connectionString)
		{
			SqlConnection connection = new SqlConnection(connectionString);
			try
			{
				connection.Open();
				connection.Close();
			}
			catch(Exception e)
			{
				Console.Error.WriteLine("Error validating connection string\n" + e.Message);
				return false;
			}
			finally
			{
				if (connection.State == System.Data.ConnectionState.Open)
					connection.Close();
			}
			return true;
		}
	}
}
