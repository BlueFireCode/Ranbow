using System;

namespace RanbowBack.Config
{
	public class Configuration
	{
		#region Singleton
		private static Configuration _instance;
		private static object padlock = new object();
		public static Configuration Instance
		{
			get
			{
				lock (padlock)
				{
					if (_instance is null)
						_instance = new();
					return _instance;
				}
			}
		}
		#endregion

		public ConfigElement Config { get; private set; }

		public ConfigElement Init()
		{
			Config = new ConfigElement
			{
				ConnectionString = Environment.GetEnvironmentVariable("ConnectionString")
			};
			return Config;
		}
	}

	public class ConfigElement
	{
		public string ConnectionString { get; internal init; }
	}
}
